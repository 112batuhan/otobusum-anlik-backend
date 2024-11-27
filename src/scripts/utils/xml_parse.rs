use serde::de::DeserializeOwned;

pub trait UnwrapSoap<R: DeserializeOwned>: DeserializeOwned {
    fn get_relevant_data(self) -> R;
}
