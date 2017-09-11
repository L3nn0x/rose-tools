error_chain!{
    links {
        RoseLib(::roselib::errors::Error, ::roselib::errors::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
