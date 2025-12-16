pub mod base;
pub mod command;
pub mod device;
pub mod families;
pub mod instance;
mod mass;
pub mod swapchain;

pub trait Store<C, I> {
    type StoredConfig;
    type StoredInfo;
}
pub struct Present;
impl<C, I> Store<C, I> for Present {
    type StoredConfig = C;
    type StoredInfo = I;
}
pub struct Absent;
impl<C, I> Store<C, I> for Absent {
    type StoredConfig = ();
    type StoredInfo = ();
}

type FieldConfig<S, C, I> = <S as Store<C, I>>::StoredConfig;
type FieldInfo<S, C, I> = <S as Store<C, I>>::StoredInfo;

pub trait Apply<For> {
    type Out;
    fn apply(self, config: For) -> Self::Out;
}
