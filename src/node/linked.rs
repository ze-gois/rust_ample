pub mod queue;

#[derive(Debug, Clone, Copy)]
pub struct LinkedNode<Observer, Reference, Allocatable>
where
    Allocatable: crate::traits::Allocatable<Observer, Reference>,
{
    pub value: Allocatable,
    pub sucessor: *const Self,
    _a: core::marker::PhantomData<Observer>,
    _b: core::marker::PhantomData<Reference>,
}
