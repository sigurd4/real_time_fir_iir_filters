use crate::{calc::iir::fourth::WahCalc, param::{CrybabyGCB95, WahFilterParam}, real_time_fir_iir_filters};

// TODO: make it SOS
crate::def_rtf!(
    WahFilter
    {
        type Param: WahFilterParam = CrybabyGCB95;

        const OUTPUTS: usize = 1;
        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 4;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let calc = WahCalc::new(param, rate);
            (
                ([], [], [
                    calc.b()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{WahFilter, CrybabyGCB95};

    #[test]
    fn plot()
    {
        let mut filter = WahFilter::new(CrybabyGCB95 {x: 0.1});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}