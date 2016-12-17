error_chain!{
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        ParseIntFieldError(value: String) {
            description("cannot parse integer field")
            display("cannot parse as integer field: '{}'", value)
        }
    }
}
