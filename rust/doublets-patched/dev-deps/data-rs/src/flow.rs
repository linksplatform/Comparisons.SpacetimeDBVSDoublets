use std::ops::{ControlFlow, FromResidual, Residual, Try};

pub enum Flow {
    Continue,
    Break,
}

impl FromResidual for Flow {
    fn from_residual(_: <Self as Try>::Residual) -> Self {
        Flow::Break
    }
}

// `Try::Residual` gained a `Residual<Self::Output>` bound on modern nightly, so `Flow`
// must also declare which `Try` type it reconstructs when used as a residual. `Flow` is
// its own residual and its `Output` is `()`, so the canonical `Try` type is `Flow` itself.
impl Residual<()> for Flow {
    type TryType = Flow;
}

impl Try for Flow {
    type Output = ();
    type Residual = Flow;

    fn from_output(_: Self::Output) -> Self {
        Flow::Continue
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Flow::Continue => ControlFlow::Continue(()),
            Flow::Break => ControlFlow::Break(Flow::Break),
        }
    }
}

impl<C, B> From<ControlFlow<C, B>> for Flow {
    fn from(flow: ControlFlow<C, B>) -> Self {
        match flow {
            ControlFlow::Continue(_) => Flow::Continue,
            ControlFlow::Break(_) => Flow::Break,
        }
    }
}
