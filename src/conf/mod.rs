use core::{fmt::Debug, marker::ConstParamTy};

moddef::moddef!(
    flat(pub) mod {
        all_pass,
        all,
        low_pass,
        band_pass,
        band_stop,
        high_pass,
        notch,
        peak
    }
);

#[derive(Clone, Copy, Debug, ConstParamTy, PartialEq, Eq)]
pub enum InputOrGND
{
    Input,
    GND,
    Either
}

impl InputOrGND
{
    pub const fn opposite(self) -> Self
    {
        match self
        {
            Self::Input => Self::GND,
            Self::GND => Self::Input,
            Self::Either => Self::Either
        }
    }

    pub const fn eq(self, rhs: InputOrGND) -> bool
    {
        matches!(
            (self, rhs),
            (Self::Input, Self::Input)
            | (Self::GND, Self::GND)
            | (Self::Either, Self::Either)
        )
    }

    pub const fn all<const N: usize>(all: [Self; N]) -> Self
    {
        assert!(N > 0);

        let y = all[0];
        let mut i = 1;
        while i < N
        {
            if !y.eq(all[i])
            {
                return Self::Either
            }
            i += 1
        }
        y
    }
}

#[derive(Clone, Copy, Debug, ConstParamTy, PartialEq, Eq)]
pub enum InputOrFeedback
{
    Input,
    Feedback,
    Either
}

impl InputOrFeedback
{
    pub const fn opposite(self) -> Self
    {
        match self
        {
            Self::Input => Self::Feedback,
            Self::Feedback => Self::Input,
            Self::Either => Self::Either
        }
    }

    pub const fn eq(self, rhs: InputOrFeedback) -> bool
    {
        matches!(
            (self, rhs),
            (Self::Input, Self::Input)
            | (Self::Feedback, Self::Feedback)
            | (Self::Either, Self::Either)
        )
    }

    pub const fn all<const N: usize>(all: [Self; N]) -> Self
    {
        assert!(N > 0);

        let y = all[0];
        let mut i = 1;
        while i < N
        {
            if !y.eq(all[i])
            {
                return Self::Either
            }
            i += 1
        }
        y
    }
}

pub enum ConfType
{
    Wildcard,
    Specific,
    Composite
}

const fn wildcard_if_zero(n: usize) -> ConfType
{
    if n == 0
    {
        ConfType::Wildcard
    }
    else
    {
        ConfType::Specific
    }
}

pub trait Conf: Sized + Copy + Debug + 'static
{
    type Wildcard: Conf<CONF_TYPE = {ConfType::Wildcard}, Wildcard = <Self as Conf>::Wildcard>;

    const CONF_TYPE: ConfType;
}

mod private
{
    use super::{All, Conf};

    pub trait _AllIfEq<C>: Conf
    {
        type _AllIfEq: Conf;
    }
    impl<C, CC> _AllIfEq<C> for CC
    where
        C: Conf,
        CC: Conf
    {
        default type _AllIfEq = CC;
    }
    impl<C> _AllIfEq<C> for C
    where
        C: Conf
    {
        type _AllIfEq = All;
    }
}