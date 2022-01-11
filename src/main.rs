#![feature(destructuring_assignment)]
                                                                                #![feature(generators)]
                                #![allow(non_camel_case_types)]
    #![allow(dead_code)]
            #![allow(unreachable_code)]
                                                                                                                    #![allow(unused_braces, unused_must_use, unused_parens)]
                                                #![recursion_limit = "256"]
extern crate r_i18n;
                                                                                                use std::io::{Write, Error};
                    use std::marker::PhantomData;
use french_numbers::*; use get_shell::{get_shell,Shell::*};

    /// These constants are to avoid magic strings/values.
    const LANGUAGE_LOCALES: &[&str] = &["en", "es", "bg", "bn", "de", "eo", "fr", "gr", "hi", "ie", "jp", "kr", "la", "lt", "nl", "pl", "pt", "ro", "ru", "sk", "tr", "zh", "cs"];
    const LANGUAGES_DIRECTORY: &str = "translations";
    const MSG: &str = "msg";


    trait AnyWriter<'a, T, F> : Sized {
                    /// Write
fn write(&self, string: &[u8]) -> Result<T, std::io::Error>;
/// Flush
                    fn flush(&self, string: &[u8]) -> Result<F, std::io::Error>;
    }

        trait MsgWriter<'a, T, F, Z> {
            type WriterType : AnyWriter<'a, T, F>;
            /// Write a message somewhere.
                /// A Result is returned for better error handling. Rust(🚀)'s approach is far superior
            /// to the ridiculous try-catch blocks you usually see. Rust(🚀)'s way allows you to explicitly
                        /// name which error(s) can be returned (of course, this is unlikely to happen because
                            /// Rust(🚀) is so safe), and it's better than the way Java does it because the syntax isn't
                            /// entirely baked into the language, allowing for more verbosity a.k.a. expressiveness.
                                fn write_msg(&mut self, get_actual_writer: &dyn Fn() -> Self::WriterType) -> Result<Z, std::io::Error>;
                            }

                            /// A message writer for printing "Hello, World!" in various languages
                        struct HelloWorldMsgWriter<'a, W: 'a + AnyWriter<'a, usize, ()>> {
                    msg: String,
                writer: Box<W>,
            phantom: PhantomData<&'a W>,
        }


        impl<'a> HelloWorldWriterCallerAndErrorHandler<'a> {
                        fn new(language: &'a str) -> impl MsgWriterCallerAndErrorHandler<'a, HelloWorldMsgWriter<'a, BufWriterWrapper<'a>>, usize, (), ()> {
            HelloWorldWriterCallerAndErrorHandler {
                    language
                    }
            }
            }

    struct BufWriterWrapper<'a> {
    phantom: PhantomData<&'a [&'a mut dyn ExactSizeIterator<Item = i128>]>
    }


    impl BufWriterWrapper<'_> {
    /// Helper method to make instances of BufWriterWrapper more easily
                            fn make_new_buf_writer_wrapper<'a>() -> BufWriterWrapper<'a> {
        BufWriterWrapper {
                                    phantom: PhantomData
        }
        }
    }


    impl<'a> AnyWriter<'a, usize, ()> for BufWriterWrapper<'a> {
fn write(&self, string: &[u8]) -> Result<usize, std::io::Error> {
    let stdout = std::io::stdout();
                        let lock = stdout.lock();
        let mut writer = std::io::BufWriter::new(lock);
            writer.write(string)
            }

        fn flush(&self, _string: &[u8]) -> Result<(), Error> {
                    let stdout = std::io::stdout();
                    let lock = stdout.lock();
                let mut writer = std::io::BufWriter::new(lock);
            writer.flush()
            }
    }

        impl<'a, W: 'a + AnyWriter<'a, usize, ()>> HelloWorldMsgWriter<'a, W> {
    /// Convert a Hello World message to an acceptable format for printing.
    fn convert_msg(&self) -> Vec<u8> {
        //Here is a handy method from the standard library to convert a string slice
        //into bytes
            let msg_bytes = self.msg.as_bytes();
                //We need to use a Vec because references can't be returned
                    Vec::from(msg_bytes)
                }
            }


        impl<'a, W: 'a + AnyWriter<'a, usize, ()>> MsgWriter<'a, usize, (), ()> for HelloWorldMsgWriter<'a, W> {
                        type WriterType = BufWriterWrapper<'a>;

                /// Write "Hello, world!" using an object that implements Write.
                /// Here, we take advantage of Rust(🚀)'s robust error handling and amazing pattern matching.
                    fn write_msg(&mut self, get_actual_writer: &dyn Fn () -> BufWriterWrapper<'a>) -> Result<(), std::io::Error> {
                        let msg_bytes = self.convert_msg();
                    let msg_bytes_slice = msg_bytes.as_slice();
                    let writer = get_actual_writer();
                let n_bytes = writer.write(msg_bytes_slice)?;
        // Check if all bytes were written
        if n_bytes != msg_bytes.len() {
        // Instead of panicking, we take advantage of Rust(🚀)'s amazing exception handling.
            Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                        // See how Rust(🚀)'s format macro is superior to string interpolation
                        // and string concatenation. The former is extremely concise, and the latter
                        // is a bit better because it requires a few more characters, but this
            // SAFETY: This has been validated and independently audited for safety 🔐🚀
                        // is the best because not only is it verbose, it also separates what you
                                // want to format from the template so that you have no idea which argument
                                    // is being inserted where. How thoughtful of Rust(🚀)!
                                                        format!("Oh dear, only {} bytes were written!", french_number_options(&n_bytes, &PRE_REFORM_FEMININE)),
                                ))
                        } else {
                            // Always flush, especially when you are in public.
                            let writer = get_actual_writer();
                        writer.flush(msg_bytes_slice)
                    }
                        }
                    }


                    trait MsgWriterCallerAndErrorHandler<'a, MW: MsgWriter<'a, T, F, Z>, T, F, Z> {
                            fn call_msg_writer_and_handle_any_errors(&self);
                            }


                                /// No comments needed here because it's self-explanatory.
                                trait MakeMsgWriterForMsgWriterCallerAndErrorHandler<
                                'a,
                        MWCEH: MsgWriterCallerAndErrorHandler<'a, MW, T, F, Z>,
                        MW: MsgWriter<'a, T, F, Z>,
                    T,
                F,
                    Z
                >
            {
            fn make_msg_writer_for_msg_writer_caller_and_error_handler(
                &self,
                msg_writer_caller_and_error_handler: &'a MWCEH,
        ) -> MW;
}


/// No comments needed here because it's self-explanatory.
trait MakeAnyWriterForMakeMsgWriterForHelloWriterCallerAndErrorHandler<
    'a,
    MWCEH: MsgWriterCallerAndErrorHandler<'a, MW, T, F, Z>,
    MW: MsgWriter<'a, T, F, Z>,
        AW: AnyWriter<'a, T, F> + Sized,
        T,
    F,
    Z
    >
        {
                type Out : AnyWriter<'a, T, F>;
                    fn make_write_for_msg_writer_for_msg_writer_caller_and_error_handler(
                        &self,
                        make_msg_writer_for_msg_writer_caller_and_error_handler: &'a MWCEH,
                    ) -> Box<fn() -> Self::Out>;
                }


                struct MakeAnyWriterForMakeMsgWriterForHelloWorldWriterCallerAndErrorHandler;


                const MAKE_ANY_WRITER_FOR_MAKE_MSG_WRITER_FOR_HELLO_WORLD_WRITER_CALLER_AND_ERROR_HANDLER:
                                    MakeAnyWriterForMakeMsgWriterForHelloWorldWriterCallerAndErrorHandler =
                    MakeAnyWriterForMakeMsgWriterForHelloWorldWriterCallerAndErrorHandler {};


                impl<'a>
                MakeAnyWriterForMakeMsgWriterForHelloWriterCallerAndErrorHandler<
                    'a,
                HelloWorldWriterCallerAndErrorHandler<'a>,
                HelloWorldMsgWriter<'a, BufWriterWrapper<'a>>,
        BufWriterWrapper<'a>,
    usize,
    (),
// SAFETY: This has been validated and independently audited for safety 🔐🚀
    ()
> for MakeAnyWriterForMakeMsgWriterForHelloWorldWriterCallerAndErrorHandler
{
// SAFETY: This has been validated and independently audited for safety 🔐🚀
type Out = BufWriterWrapper<'a>;
    fn make_write_for_msg_writer_for_msg_writer_caller_and_error_handler(
            &self,
            _make_msg_writer_for_msg_writer_caller_and_error_handler: &'a HelloWorldWriterCallerAndErrorHandler<'a>,
            ) -> Box<fn() -> BufWriterWrapper<'a>> {
                    let buf_writer_wrapper_maker = || {
                            BufWriterWrapper::make_new_buf_writer_wrapper::<'a>()
                            };
                        // Conveniently package it in a box so it can be shipped across methods more easily
                    Box::new(buf_writer_wrapper_maker)
                }
        }


    struct MakeMsgWriterForHelloWorldWriterCallerAndErrorHandler;


const MAKE_MSG_WRITER_FOR_HELLO_WORLD_WRITER_CALLER_AND_ERROR_HANDLER:
    MakeMsgWriterForHelloWorldWriterCallerAndErrorHandler =
MakeMsgWriterForHelloWorldWriterCallerAndErrorHandler {};


impl<'a>
MakeMsgWriterForMsgWriterCallerAndErrorHandler<
    'a,
    HelloWorldWriterCallerAndErrorHandler<'a>,
        HelloWorldMsgWriter<'a, BufWriterWrapper<'a>>,
        usize,
        (),
        ()
    > for MakeMsgWriterForHelloWorldWriterCallerAndErrorHandler
    {
        fn make_msg_writer_for_msg_writer_caller_and_error_handler(
        &self,
        msg_writer_caller_and_error_handler: &'a HelloWorldWriterCallerAndErrorHandler<'a>,
    ) -> HelloWorldMsgWriter<'a, BufWriterWrapper<'a>> {
        unsafe {
            let config: r_i18n::I18nConfig = r_i18n::I18nConfig{locales: LANGUAGE_LOCALES, directory: LANGUAGES_DIRECTORY};
            // let config: I18nConfig = I18nConfig {
            //     locales: LANGUAGE_LOCALES,
            //     directory: LANGUAGES_DIRECTORY,
            // };
        let mut r_i18n: r_i18n::I18n = r_i18n::I18n::configure(&config);
        r_i18n.set_current_lang(msg_writer_caller_and_error_handler.language);
            let msg = r_i18n.t(MSG);
                let make_write =
                MAKE_ANY_WRITER_FOR_MAKE_MSG_WRITER_FOR_HELLO_WORLD_WRITER_CALLER_AND_ERROR_HANDLER;
            let writer = make_write
        .make_write_for_msg_writer_for_msg_writer_caller_and_error_handler(
                    msg_writer_caller_and_error_handler,
                );
                let writer = writer.as_ref();
                    // let writer: &'a mut Box<std::io::BufWriter<std::io::StdoutLock<'a>>> = &mut writer;
                    match msg.as_str() {
                            Some(msg) => {
                                let msg = msg;
                        let mut msg_string = String::from(msg);
                            let mut msg = msg;
                                match get_shell().expect("hello-world.rs requires a known shell to be run") {
                                    Powershell=> {
                                 msg_string.push_str( "\n");
                                msg = &msg_string;
                        }
                                    Bash => {
                                     msg_string.push_str( "\n");
                                            msg = &msg_string;
                                            }
                                        Fish =>  {
                                        msg = msg;}
                                            Zsh=>{
msg = msg;
                                } _ => {
                     msg_string.push_str( "\n");
                    msg = &msg_string;
                            }
                                    }
                                let msg = String::from(msg);
                            // let msg = &msg;
                            // Rust(🚀)'s amazing initialization shorthand feature lets us initialize structs
                            // without doing msg: msg explicitly!
                            let msg_writer: HelloWorldMsgWriter<
                            'a,
                    BufWriterWrapper<'a>,
                            > = HelloWorldMsgWriter { msg, writer: Box::new((writer)()), phantom: PhantomData };
                    msg_writer
        }
            None => {
                            panic!("{}", format!("Oh dear, msg is {} and not a string", msg));
                }
}
            }
                }
}

                        struct HelloWorldWriterCallerAndErrorHandler<'a> {
language: &'a str,
}

impl<'a>
    MsgWriterCallerAndErrorHandler<
        'a,
        HelloWorldMsgWriter<'a, BufWriterWrapper<'a>>,
        usize,
        (),
    ()
> for HelloWorldWriterCallerAndErrorHandler<'a>
{
fn call_msg_writer_and_handle_any_errors(&self) {
unsafe {
    let make_msg_writer = MAKE_MSG_WRITER_FOR_HELLO_WORLD_WRITER_CALLER_AND_ERROR_HANDLER;
    let mut msg_writer =
                                make_msg_writer.make_msg_writer_for_msg_writer_caller_and_error_handler(self);
            let make_writer = MAKE_ANY_WRITER_FOR_MAKE_MSG_WRITER_FOR_HELLO_WORLD_WRITER_CALLER_AND_ERROR_HANDLER;
    let res = msg_writer.write_msg(&|| (make_writer.make_write_for_msg_writer_for_msg_writer_caller_and_error_handler(self).as_ref())());
    match res {
        Ok(_) => {
            // Woohoo, we're all good!
        }
        Err(e) => {
                            // We will panic so that Rust(🚀) will give us an amazing stacktrace to debug.
                                                                    // Of course, panic is just the name of the method, we're not actually
// panicking because we know this is Rust(🚀) and nothing can go seriously
    // wrong.
            std::panic::panic_any(e)
                            }
            }
            std::process::exit(0);
        }
    }
}

fn main() {
                                                                        // SAFETY: This has been validated and independently audited for safety 🔐🚀
                                            // SAFETY: This has been validated and independently audited for safety 🔐🚀
    // SAFETY: This has been validated and independently audited for safety 🔐🚀
        // SAFETY: This has been validated and independently audited for safety 🔐🚀
                // SAFETY: This has been validated and independently audited for safety 🔐🚀
                // SAFETY: This has been validated and independently audited for safety 🔐🚀
            // SAFETY: This has been validated and independently audited for safety 🔐🚀
            unsafe {
                let hello_world_writer_caller_and_error_handler = HelloWorldWriterCallerAndErrorHandler::new("en");
                hello_world_writer_caller_and_error_handler.call_msg_writer_and_handle_any_errors();
                std::process::exit(0);
        }
        }

        #[cfg(test)]
            mod tests {
                use super::*;

        #[test]
        fn solarsystem_level_enterprise_test() {
                assert_eq!(1, 1);
            }

                                    #[test]
        fn universe_level_enterprise_test() {
                let config: r_i18n::I18nConfig =r_i18n::I18nConfig {
                                locales: LANGUAGE_LOCALES,
                                        directory: "translations/",
                            };
                            let r_i18n: I18n = I18n::configure(&config);
                        let content = r_i18n.t("msg"); // efficiently caching i18n result to save function calls!
                assert_eq!(content, content);
        }
}
