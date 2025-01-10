use termcolor;

#[derive(Debug)]
pub enum ChartStyle {
    Basic,    // █
    Blocks,   // ▏▎▍▌▋▊▉█
    Dots,     // ⠄⠆⠖⠶⢶⣶⣾⣿
    Lines,    // ─━
}

#[derive(Debug)]
pub struct ChartConfig {
    pub style: ChartStyle,
    pub color: Option<termcolor::Color>,
    pub width: usize,
    pub show_percentage: bool,
}

pub fn create_enhanced_chart(data: &[(String, usize)], config: ChartConfig) -> String {
    let total_value: usize = data.iter().map(|(_, v)| *v).sum();
    let mut chart = String::new();

    for (label, value) in data {
        let percentage = (*value as f64 / total_value as f64) * 100.0;
        let bar_width = ((config.width as f64 * percentage) / 100.0) as usize;
        
        let bar = match config.style {
            ChartStyle::Basic => "█".repeat(bar_width),
            ChartStyle::Blocks => {
                let blocks = ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
                let full_blocks = "█".repeat(bar_width / 8);
                let remainder = bar_width % 8;
                if remainder > 0 {
                    format!("{}{}", full_blocks, blocks[remainder - 1])
                } else {
                    full_blocks
                }
            },
            ChartStyle::Dots => {
                let dots = ["⠄", "⠆", "⠖", "⠶", "⢶", "⣶", "⣾", "⣿"];
                let full_dots = "⣿".repeat(bar_width / 8);
                let remainder = bar_width % 8;
                if remainder > 0 {
                    format!("{}{}", full_dots, dots[remainder - 1])
                } else {
                    full_dots
                }
            },
            ChartStyle::Lines => {
                let full_lines = "━".repeat(bar_width);
                if percentage < 100.0 {
                    format!("{}─", full_lines)
                } else {
                    full_lines
                }
            },
        };

        let line = if config.show_percentage {
            format!("{:<20} [{:>3.0}%] {}\n", label, percentage, bar)
        } else {
            format!("{:<20} {}\n", label, bar)
        };

        chart.push_str(&line);
    }

    chart
}
