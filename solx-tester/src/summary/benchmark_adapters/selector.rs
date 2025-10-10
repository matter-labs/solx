//!
//! Converts `[TestSelector]` to the representation used by the benchmark.
//!

use crate::test::selector::TestSelector;

impl From<TestSelector> for solx_benchmark_converter::TestSelector {
    fn from(selector: TestSelector) -> Self {
        let TestSelector { path, case, input } = selector;
        let input = input.map(Into::into);
        solx_benchmark_converter::TestSelector {
            project: path,
            case,
            input,
        }
    }
}
