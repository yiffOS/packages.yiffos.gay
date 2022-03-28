table! {
    packages (name) {
        name -> Text,
        version -> Text,
        epoch -> Int4,
        description -> Text,
        groups -> Nullable<Array<Text>>,
        url -> Text,
        license -> Nullable<Array<Text>>,
        depends -> Nullable<Array<Text>>,
        optional_depends -> Nullable<Array<Text>>,
        make_depends -> Nullable<Array<Text>>,
        provides -> Array<Text>,
        conflicts -> Nullable<Array<Text>>,
        replaces -> Nullable<Array<Text>>,
        maintainers -> Array<Text>,
        repo -> Text,
    }
}
