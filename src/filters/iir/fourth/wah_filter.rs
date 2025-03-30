use crate::{calc::iir::fourth::WahCalc, param::{CrybabyGCB95, WahFilterParam}, real_time_fir_iir_filters};

// TODO: make it SOS
crate::def_rtf!(
    {
        /// # Configuration
        /// 
        /// <pre>
        ///                  V꜀꜀
        ///                   |
        ///                 [R꜀₁]
        ///                   |
        ///                   o-----o--------------------o----------o    V꜀꜀
        ///                   |     |                    |          |     |
        ///                   /    [Rⱼ]                 [C∞]       [Rⱼ] [R꜀₂]
        ///                 |/      |                    |          |     |
        /// X-[Cᵢ]-[Rᵢ]-o---| β     |                Y---o          |     /
        ///             |   |\      |                    |   x      |   |/
        ///            [Rₛ]   v     o----o----o----o   [Rₚₒₜ]<-[C∞]-o---| β
        ///             |     |     |    |    |    |     |              |\
        ///             |   [Rₑ₁]  [L]  [Rₚ] [R₉] [C₉]   V                v
        ///             |     |     |    |    |    |                      |
        ///             |     ⏚     |    |    ⏚    ⏚                      |
        ///             |           |    |                                |
        ///             o-----------o----o---------------------------[Cբ]-o
        ///                                                               |
        ///                                                             [Rₑ₂]
        ///                                                               |
        ///                                                               ⏚
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// x = 0.3
        /// 
        /// ## Output
        /// 
        /// <div>
        /// <img alt="Wah filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/wah_filter0.png" height="500">
        /// </div>
    }
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
        let mut filter = WahFilter::new(CrybabyGCB95 {x: 0.3});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}