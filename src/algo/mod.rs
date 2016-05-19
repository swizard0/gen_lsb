use par_exec::Executor;

pub mod mu_comma_lambda;

pub trait Algorithm {
    type Exec: Executor;
    type Res;
    type Err;

    fn run(executor: Self::Exec) -> Result<Self::Res, Self::Err>;
}
