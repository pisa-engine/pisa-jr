//! Module related to parsing TREC collection format.

use crate::{skip_whitespaces, Document};
use itertools::Itertools;
use std::io::{self, BufRead, BufReader, Cursor, Read};

//pub fn parse_collection<R>(reader: R) -> impl Iterator<Item = io::Result<Document>>
//where
//    R: BufRead,
//{
//    reader.bytes().peekable().batching(|bytes| {
//        let mut pattern = "</DOC>";
//        let mut pos = 0;
//        let mut buf: Vec<u8> = Vec::new();
//        while let Some(byte) = bytes.next() {
//            if let Ok(byte) = byte {
//                match (byte, pos) {
//                    (b'<', 0) | (b'/', 1) | (b'D', 2) | (b'O', 3) | (b'C', 4) | (b'>', 5) => {
//                        pos += 1
//                    }
//                    _ => pos = 0,
//                }
//                if pos == 6 {
//                    return Some(Ok(buf));
//                }
//            } else {
//                return Some(Err(""));
//            }
//        }
//        None
//    });
//    todo!()
//}

/// Represents the trailing bytes of a TREC record: `</DOC>`.
const DOC_CLOSING_TAG: [u8; 6] = [b'<', b'/', b'D', b'O', b'C', b'>'];

/// # Example
///
///
pub fn split_trec_records<R>(reader: R) -> impl Iterator<Item = Result<Vec<u8>, io::Error>>
where
    R: BufRead,
{
    reader
        .bytes()
        .peekable()
        .batching(|bytes| -> Option<Result<_, _>> {
            skip_whitespaces(bytes);
            bytes.next().map(|first_byte| {
                let read_rest = move || -> Result<_, _> {
                    let mut buf: Vec<u8> = vec![first_byte?];
                    let mut pos = 0;
                    while let Some(byte) = bytes.next() {
                        match byte {
                            Ok(byte) => {
                                buf.push(byte);
                                if DOC_CLOSING_TAG[pos] == byte {
                                    pos += 1;
                                } else {
                                    pos = 0;
                                }
                                if pos == 6 {
                                    break;
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    Ok(buf)
                };
                read_rest()
            })
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_trec_records() {
        let input = r#"
<DOC> <DOCNO> 0 </DOCNO> zero </DOC>
<DOC> <DOCNO> 1 </DOCNO> ten </DOC>
<DOC> <DOCNO> 2 </DOCNO> ten nine </DOC>
    "#;
        let records: Result<Vec<_>, _> = split_trec_records(Cursor::new(input))
            .map(|bytes| String::from_utf8(bytes.unwrap()))
            .collect();
        assert_eq!(
            records.unwrap(),
            vec![
                "<DOC> <DOCNO> 0 </DOCNO> zero </DOC>".to_string(),
                "<DOC> <DOCNO> 1 </DOCNO> ten </DOC>".to_string(),
                "<DOC> <DOCNO> 2 </DOCNO> ten nine </DOC>".to_string(),
            ]
        );
    }

    //    #[test]
    //    fn test_parse_single_document() -> io::Result<()> {
    //        let input = "<DOC> <DOCNO> 0 </DOCNO> zero </DOC>";
    //        let mut documents = parse_collection(BufReader::new(Cursor::new(input)));
    //        assert_eq!(
    //            documents.next().expect("document.next() is None. Oops.")?,
    //            Document::new("0", "zero")
    //        );
    //        Ok(())
    //    }
    //
    //    #[test]
    //    fn test_parse_multiple_documents() -> io::Result<()> {
    //        let input = r#"
    //<DOC> <DOCNO> 0 </DOCNO> zero </DOC>
    //<DOC> <DOCNO> 1 </DOCNO> ten </DOC>
    //<DOC> <DOCNO> 2 </DOCNO> ten nine </DOC>
    //
    //<DOC> <DOCNO> 3 </DOCNO> ten nine eight </DOC>
    //<DOC>
    //<DOCNO> 4 </DOCNO>
    //
    //ten nine eight seven
    //
    //</DOC>
    //
    //<DOC> <DOCNO> 5 </DOCNO> ten nine eight seven six </DOC>
    //<DOC> <DOCNO> 6 </DOCNO> ten nine eight seven six five </DOC>
    //<DOC> <DOCNO> 7 </DOCNO> ten nine eight seven six five four </DOC>
    //<DOC> <DOCNO> 8 </DOCNO> ten nine eight seven six five four three </DOC>
    //<DOC> <DOCNO> 9 </DOCNO> ten nine eight seven six five four three two </DOC>
    //<DOC> <DOCNO> 10 </DOCNO> ten nine eight seven six five four three two one </DOC>
    //            "#;
    //        let documents: io::Result<Vec<_>> =
    //            parse_collection(BufReader::new(Cursor::new(input))).collect();
    //        assert!(documents.is_ok());
    //        assert_eq!(
    //            documents.unwrap(),
    //            vec![
    //                Document::new("0", "zero"),
    //                Document::new("1", "ten"),
    //                Document::new("2", "ten nine"),
    //                Document::new("3", "ten nine eight"),
    //                Document::new("4", "ten nine eight seven"),
    //                Document::new("5", "ten nine eight seven six"),
    //                Document::new("6", "ten nine eight seven six five"),
    //                Document::new("7", "ten nine eight seven six five four"),
    //                Document::new("8", "ten nine eight seven six five four three"),
    //                Document::new("9", "ten nine eight seven six five four three two"),
    //                Document::new("10", "ten nine eight seven six five four three two one"),
    //            ]
    //        );
    //        Ok(())
    //    }
}
