#[macro_export]
macro_rules! declare_subtype {
    ($enum_name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(
            Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
            Encode, Decode, TypeInfo, MaxEncodedLen
        )]
        pub enum $enum_name {
            $($variant),*
        }
    }
}

#[macro_export]
macro_rules! declare_music_genre {
    ($($genre:ident($subtype:ident { $($variant:ident),* $(,)? }),)*) => {
        $(declare_subtype!($subtype { $($variant),* });)*

        #[derive(
            Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
            Encode, Decode, TypeInfo, MaxEncodedLen
        )]
        pub enum MusicGenre {
            $($genre(Option<$subtype>)),*
        }
    }
}
