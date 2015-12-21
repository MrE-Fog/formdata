// Copyright © 2015 by Michael Dilger (of New Zealand)
// This code is licensed under the MIT license (see LICENSE-MIT for details)

use std::path::PathBuf;
use std::ops::Drop;
use mime::Mime;
use tempdir::TempDir;
use error::Error;
use textnonce::TextNonce;

/// An uploaded file that was received as part of `multipart/form-data` parsing.
///
/// Files are streamed to disk because they may not fit in memory.
#[derive(Clone, Debug, PartialEq)]
pub struct UploadedFile {
    /// The temporary file where the data was saved.
    pub path: PathBuf,
    /// The filename that was specified in the data, unfiltered. It may or may not be legal on the
    /// local filesystem.
    pub filename: Option<String>,
    /// The unvalidated content-type that was specified in the data.
    pub content_type: Mime,
    /// The size of the file.
    pub size: usize,
    // The temporary directory the upload was put into, saved for the Drop trait
    tempdir: PathBuf,
}

impl UploadedFile {
    pub fn new(content_type: Mime) -> Result<UploadedFile,Error> {
        // Setup a file to capture the contents.
        let tempdir = try!(TempDir::new("formdata")).into_path();
        let mut path = tempdir.clone();
        path.push(TextNonce::sized_urlsafe(32).unwrap().into_string());
        Ok(UploadedFile {
            path: path,
            filename: None,
            content_type: content_type,
            size: 0,
            tempdir: tempdir,
        })
    }
}

impl Drop for UploadedFile {
    fn drop(&mut self) {
        let _ = ::std::fs::remove_file(&self.path);
        let _ = ::std::fs::remove_dir(&self.tempdir);
    }
}

// This was generated by serde_codegen.  Due to a bug in serde_codegen, we include
// and commit its generated output for the time being.
impl ::serde::de::Deserialize for UploadedFile {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<UploadedFile, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __field2, __field3, __field4, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_usize<E>(&mut self, value: usize)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                0usize => { Ok(__Field::__field0) }
                                1usize => { Ok(__Field::__field1) }
                                2usize => { Ok(__Field::__field2) }
                                3usize => { Ok(__Field::__field3) }
                                4usize => { Ok(__Field::__field4) }
                                _ => {
                                    Err(::serde::de::Error::syntax("expected a field"))
                                }
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "path" => { Ok(__Field::__field0) }
                                "filename" => { Ok(__Field::__field1) }
                                "content_type" => { Ok(__Field::__field2) }
                                "size" => { Ok(__Field::__field3) }
                                "tempdir" => { Ok(__Field::__field4) }
                                _ => {
                                    Err(::serde::de::Error::unknown_field(value))
                                }
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match ::std::str::from_utf8(value) {
                                Ok(s) => self.visit_str(s),
                                _ => {
                                    Err(::serde::de::Error::syntax("could not convert a byte string to a String"))
                                }
                            }
                        }
                    }
                    deserializer.visit(__FieldVisitor::<D>{phantom:
                                                               PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                UploadedFile;
                #[inline]
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<UploadedFile, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field2 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field3 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field4 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(UploadedFile{path: __field0,
                                        filename: __field1,
                                        content_type: __field2,
                                        size: __field3,
                                        tempdir: __field4,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<UploadedFile, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        let mut __field3 = None;
                        let mut __field4 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!(visitor . visit_value (
                                                  )));
                                }
                                __Field::__field1 => {
                                    __field1 =
                                        Some(try!(visitor . visit_value (
                                                  )));
                                }
                                __Field::__field2 => {
                                    __field2 =
                                        Some(try!(visitor . visit_value (
                                                  )));
                                }
                                __Field::__field3 => {
                                    __field3 =
                                        Some(try!(visitor . visit_value (
                                                  )));
                                }
                                __Field::__field4 => {
                                    __field4 =
                                        Some(try!(visitor . visit_value (
                                                  )));
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "path" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "filename" )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "content_type"
                                     )),
                            };
                        let __field3 =
                            match __field3 {
                                Some(__field3) => __field3,
                                None =>
                                try!(visitor . missing_field ( "size" )),
                            };
                        let __field4 =
                            match __field4 {
                                Some(__field4) => __field4,
                                None =>
                                try!(visitor . missing_field ( "tempdir" )),
                            };
                        try!(visitor . end (  ));
                        Ok(UploadedFile{path: __field0,
                                        filename: __field1,
                                        content_type: __field2,
                                        size: __field3,
                                        tempdir: __field4,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] =
                &["path", "filename", "content_type", "size", "tempdir"];
            deserializer.visit_struct("UploadedFile", FIELDS,
                                      __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
impl ::serde::ser::Serialize for UploadedFile {
    fn serialize<__S>(&self, serializer: &mut __S)
     -> ::std::result::Result<(), __S::Error> where
     __S: ::serde::ser::Serializer {
        {
            struct Visitor<'__a> {
                state: usize,
                value: &'__a UploadedFile,
                _structure_ty: ::std::marker::PhantomData<&'__a UploadedFile>,
            }
            impl <'__a> ::serde::ser::MapVisitor for Visitor<'__a> {
                #[inline]
                fn visit<S>(&mut self, serializer: &mut S)
                 -> ::std::result::Result<Option<()>, S::Error> where
                 S: ::serde::ser::Serializer {
                    loop  {
                        match self.state {
                            0usize => {
                                self.state += 1;
                                { }
                                return Ok(Some(try!(serializer .
                                                    visit_struct_elt (
                                                    "path" , &self.value.path
                                                    , ))));
                            }
                            1usize => {
                                self.state += 1;
                                { }
                                return Ok(Some(try!(serializer .
                                                    visit_struct_elt (
                                                    "filename" ,
                                                    &self.value.filename ,
                                                    ))));
                            }
                            2usize => {
                                self.state += 1;
                                { }
                                return Ok(Some(try!(serializer .
                                                    visit_struct_elt (
                                                    "content_type" ,
                                                    &self.value.content_type ,
                                                    ))));
                            }
                            3usize => {
                                self.state += 1;
                                { }
                                return Ok(Some(try!(serializer .
                                                    visit_struct_elt (
                                                    "size" , &self.value.size
                                                    , ))));
                            }
                            4usize => {
                                self.state += 1;
                                { }
                                return Ok(Some(try!(serializer .
                                                    visit_struct_elt (
                                                    "tempdir" ,
                                                    &self.value.tempdir ,
                                                    ))));
                            }
                            _ => { return Ok(None); }
                        }
                    }
                }
                #[inline]
                fn len(&self) -> Option<usize> { Some(0 + 1 + 1 + 1 + 1 + 1) }
            }
            serializer.visit_struct("UploadedFile",
                                    Visitor{value: self,
                                            state: 0,
                                            _structure_ty:
                                                ::std::marker::PhantomData::<&UploadedFile>,})
        }
    }
}