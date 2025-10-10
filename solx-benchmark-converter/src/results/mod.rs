//!
//! The benchmark group results.
//!

pub mod group;
pub mod run_description;

use colored::Colorize;
use run_description::RunDescription;
use std::cmp;

///
/// The benchmark group results.
///
#[derive(Debug)]
pub struct Results<'a> {
    /// The size best result.
    pub size_best: f64,
    /// The size worst result.
    pub size_worst: f64,
    /// The size total decrease result.
    pub size_total: f64,
    /// The size negative result test names.
    pub size_negatives: Vec<(f64, RunDescription<'a>)>,
    /// The size positive result test names.
    pub size_positives: Vec<(f64, RunDescription<'a>)>,

    /// The cycles best result.
    pub cycles_best: f64,
    /// The cycles worst result.
    pub cycles_worst: f64,
    /// The cycles total decrease result.
    pub cycles_total: f64,
    /// The cycles negative result test names.
    pub cycles_negatives: Vec<(f64, RunDescription<'a>)>,
    /// The cycles positive result test names.
    pub cycles_positives: Vec<(f64, RunDescription<'a>)>,

    /// The ergs best result.
    pub ergs_best: f64,
    /// The ergs worst result.
    pub ergs_worst: f64,
    /// The ergs total decrease result.
    pub ergs_total: f64,
    /// The ergs negative result test names.
    pub ergs_negatives: Vec<(f64, RunDescription<'a>)>,
    /// The ergs positive result test names.
    pub ergs_positives: Vec<(f64, RunDescription<'a>)>,

    /// The gas best result.
    pub gas_best: f64,
    /// The gas worst result.
    pub gas_worst: f64,
    /// The gas total decrease result.
    pub gas_total: f64,
    /// The gas negative result test names.
    pub gas_negatives: Vec<(f64, RunDescription<'a>)>,
    /// The gas positive result test names.
    pub gas_positives: Vec<(f64, RunDescription<'a>)>,

    /// The EVM interpreter reference ratios.
    pub evm_interpreter_reference_ratios: Option<Vec<(String, f64)>>,
    /// The EVM interpreter candidate ratios.
    pub evm_interpreter_candidate_ratios: Option<Vec<(String, f64)>>,
}

impl<'a> Results<'a> {
    ///
    /// A shortcut constructor.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        size_best: f64,
        size_worst: f64,
        size_total: f64,
        size_negatives: Vec<(f64, RunDescription<'a>)>,
        size_positives: Vec<(f64, RunDescription<'a>)>,

        cycles_best: f64,
        cycles_worst: f64,
        cycles_total: f64,
        cycles_negatives: Vec<(f64, RunDescription<'a>)>,
        cycles_positives: Vec<(f64, RunDescription<'a>)>,

        ergs_best: f64,
        ergs_worst: f64,
        ergs_total: f64,
        ergs_negatives: Vec<(f64, RunDescription<'a>)>,
        ergs_positives: Vec<(f64, RunDescription<'a>)>,

        gas_best: f64,
        gas_worst: f64,
        gas_total: f64,
        gas_negatives: Vec<(f64, RunDescription<'a>)>,
        gas_positives: Vec<(f64, RunDescription<'a>)>,
    ) -> Self {
        Self {
            size_best,
            size_worst,
            size_total,
            size_negatives,
            size_positives,

            cycles_best,
            cycles_worst,
            cycles_total,
            cycles_negatives,
            cycles_positives,

            ergs_best,
            ergs_worst,
            ergs_total,
            ergs_negatives,
            ergs_positives,

            gas_best,
            gas_worst,
            gas_total,
            gas_negatives,
            gas_positives,

            evm_interpreter_reference_ratios: None,
            evm_interpreter_candidate_ratios: None,
        }
    }

    ///
    /// Sets the EVM interpreter ratios.
    ///
    pub fn set_evm_interpreter_ratios(
        &mut self,
        reference_ratios: Vec<(String, f64)>,
        candidate_ratios: Vec<(String, f64)>,
    ) {
        self.evm_interpreter_reference_ratios = Some(reference_ratios);
        self.evm_interpreter_candidate_ratios = Some(candidate_ratios);
    }

    ///
    /// Sorts the worst results.
    ///
    pub fn sort_worst(&mut self) {
        self.size_negatives.sort_by(|a, b| {
            if a.0 > b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        self.cycles_negatives.sort_by(|a, b| {
            if a.0 > b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        self.ergs_negatives.sort_by(|a, b| {
            if a.0 > b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        self.gas_negatives.sort_by(|a, b| {
            if a.0 > b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        self.size_positives.sort_by(|a, b| {
            if a.0 < b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        self.cycles_positives.sort_by(|a, b| {
            if a.0 < b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        self.ergs_positives.sort_by(|a, b| {
            if a.0 < b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        self.gas_positives.sort_by(|a, b| {
            if a.0 < b.0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    ///
    /// Writes the top benchmark results top to the terminal.
    ///
    pub fn print_top_results(&self, count: usize, group_name: &str) {
        println!(
            "Group '{}' size (-%) worst {group_name} out of {count}:",
            self.size_negatives.len()
        );
        for (value, entry) in self.size_negatives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
        println!(
            "Group '{}' cycles (-%) worst {group_name} out of {count}:",
            self.cycles_negatives.len()
        );
        for (value, entry) in self.cycles_negatives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
        println!(
            "Group '{}' ergs (-%) worst {group_name} out of {count}:",
            self.ergs_negatives.len()
        );
        for (value, entry) in self.ergs_negatives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
        println!(
            "Group '{}' gas (-%) worst {group_name} out of {count}:",
            self.gas_negatives.len()
        );
        for (value, entry) in self.gas_negatives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();

        println!(
            "Group '{}' size (-%) best {group_name} out of {count}:",
            self.size_positives.len()
        );
        for (value, entry) in self.size_positives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
        println!(
            "Group '{}' cycles (-%) best {group_name} out of {count}:",
            self.cycles_positives.len()
        );
        for (value, entry) in self.cycles_positives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
        println!(
            "Group '{}' ergs (-%) best {group_name} out of {count}:",
            self.ergs_positives.len()
        );
        for (value, entry) in self.ergs_positives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
        println!(
            "Group '{}' gas (-%) best {group_name} out of {count}:",
            self.gas_positives.len()
        );
        for (value, entry) in self.gas_positives.iter().take(count) {
            println!("{:010}: {}", Self::format_f64(*value), entry);
        }
        println!();
    }

    ///
    /// Prints the results to a file.
    ///
    pub fn write_all<W>(&self, w: &mut W, group_name: &str) -> anyhow::Result<()>
    where
        W: std::io::Write,
    {
        writeln!(
            w,
            "╔═╡ {} ╞{}╡ {} ╞═╗",
            "Size (-%)".bright_white(),
            "═".repeat(cmp::max(34 - group_name.len(), 0)),
            group_name.bright_white()
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Best".bright_white(),
            Self::format_f64(self.size_best)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Worst".bright_white(),
            Self::format_f64(self.size_worst)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Total".bright_white(),
            Self::format_f64(self.size_total)
        )?;

        writeln!(
            w,
            "╠═╡ {} ╞{}╡ {} ╞═╣",
            "Cycles (-%)".bright_white(),
            "═".repeat(cmp::max(32 - group_name.len(), 0)),
            group_name.bright_white()
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Best".bright_white(),
            Self::format_f64(self.cycles_best)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Worst".bright_white(),
            Self::format_f64(self.cycles_worst)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Total".bright_white(),
            Self::format_f64(self.cycles_total)
        )?;

        writeln!(
            w,
            "╠═╡ {} ╞{}╡ {} ╞═╣",
            "Ergs (-%)".bright_white(),
            "═".repeat(cmp::max(34 - group_name.len(), 0)),
            group_name.bright_white()
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Best".bright_white(),
            Self::format_f64(self.ergs_best)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Worst".bright_white(),
            Self::format_f64(self.ergs_worst)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Total".bright_white(),
            Self::format_f64(self.ergs_total)
        )?;

        writeln!(
            w,
            "╠══╡ {} ╞{}╡ {} ╞═╣",
            "Gas (-%)".bright_white(),
            "═".repeat(cmp::max(34 - group_name.len(), 0)),
            group_name.bright_white()
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Best".bright_white(),
            Self::format_f64(self.gas_best)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Worst".bright_white(),
            Self::format_f64(self.gas_worst)
        )?;
        writeln!(
            w,
            "║ {:43} {:07} ║",
            "Total".bright_white(),
            Self::format_f64(self.gas_total)
        )?;

        if let (Some(gas_reference_ratios), Some(gas_candidate_ratios)) = (
            self.evm_interpreter_reference_ratios.as_deref(),
            self.evm_interpreter_candidate_ratios.as_deref(),
        ) {
            writeln!(
                w,
                "╠═╡ {} ╞{}╡ {} ╞═╣",
                "Ergs/gas".bright_white(),
                "═".repeat(cmp::max(35 - group_name.len(), 0)),
                group_name.bright_white()
            )?;
            for (opcode, reference_ratio) in gas_reference_ratios.iter() {
                let reference_ratio = *reference_ratio;
                let candidate_ratio = gas_candidate_ratios
                    .iter()
                    .find_map(|(key, value)| {
                        if key.as_str() == opcode.as_str() {
                            Some(*value)
                        } else {
                            None
                        }
                    })
                    .expect("Always exists");
                let is_positive = candidate_ratio < reference_ratio;
                let is_negative = candidate_ratio > reference_ratio;

                writeln!(
                    w,
                    "║ {:42} {} ║",
                    if is_positive {
                        opcode.green()
                    } else if is_negative {
                        opcode.bright_red()
                    } else {
                        opcode.bright_white()
                    },
                    if is_positive {
                        format!("{candidate_ratio:8.3}").green()
                    } else if is_negative {
                        format!("{candidate_ratio:8.3}").bright_red()
                    } else {
                        format!("{candidate_ratio:8.3}").bright_white()
                    },
                )?;
            }

            writeln!(
                w,
                "╠═╡ {} ╞{}╡ {} ╞═╣",
                "Ergs/gas (-%)".bright_white(),
                "═".repeat(cmp::max(30 - group_name.len(), 0)),
                group_name.bright_white()
            )?;
            for (opcode, reference_ratio) in gas_reference_ratios.iter() {
                let reference_ratio = *reference_ratio;
                let candidate_ratio = gas_candidate_ratios
                    .iter()
                    .find_map(|(key, value)| {
                        if key.as_str() == opcode.as_str() {
                            Some(*value)
                        } else {
                            None
                        }
                    })
                    .expect("Always exists");

                let reduction = 100.0 - (candidate_ratio * 100.0 / reference_ratio);
                if reduction.abs() >= 0.001 {
                    let is_positive = candidate_ratio < reference_ratio;
                    let is_negative = candidate_ratio > reference_ratio;

                    writeln!(
                        w,
                        "║ {:42} {} ║",
                        if is_positive {
                            opcode.green()
                        } else if is_negative {
                            opcode.bright_red()
                        } else {
                            opcode.bright_white()
                        },
                        if is_positive {
                            format!("{reduction:8.3}").green()
                        } else if is_negative {
                            format!("{reduction:8.3}").bright_red()
                        } else {
                            format!("{reduction:8.3}").bright_white()
                        },
                    )?;
                }
            }
        }
        writeln!(w, "╚{}╝", "═".repeat(53),)?;

        Ok(())
    }

    ///
    /// Formats and colorizes an `f64` value.
    ///
    fn format_f64(value: f64) -> colored::ColoredString {
        if value > 1.0 {
            format!("{:7.3}", 100.0 - value * 100.0).bright_red()
        } else if value == 1.0 {
            format!("{:7.3}", 100.0 - value * 100.0).white()
        } else {
            format!("{:7.3}", 100.0 - value * 100.0).green()
        }
    }
}
