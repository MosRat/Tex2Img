#![allow(unused)]
use anyhow::{Ok, Result, anyhow};
use clap::{Parser, arg, command};
use pdfium_render::prelude::*;
use std::path::PathBuf;
use tectonic;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Tex to gen (csv)
    #[arg(short, long, value_name = "LETEX")]
    latex: String,

    /// output file name
    #[arg(short, long, value_name = "INPUT")]
    name: String,

    /// Output dir
    #[arg(short, long, value_name = "OUTPUT")]
    output: PathBuf,

    #[arg(long, short, value_name = "WIDTH", default_value_t = 448)]
    width: u32,
}

// const HEAD: &str = r#"
// \documentclass[margin={1cm,1cm,1cm,1cm},varwidth=15cm]{standalone}
// \usepackage[utf8]{inputenc}
// \usepackage{fontspec}
// \usepackage{ctex}
// \usepackage{amsmath}
// \usepackage{amsfonts}
// \usepackage{amssymb}
// \begin{document}
// "#;

// const TAIL: &str = r#"

// \end{document}
// "#;

const HEAD: &str = r#"
\documentclass[multi={mathpage},border=2pt, varwidth]{standalone}
\usepackage{amsmath, amssymb, amsfonts}
\usepackage{fontspec}
\usepackage{ctex}


\usepackage{xcolor} % 白底黑字增强对比度
\newenvironment{mathpage}{}{}
\begin{document}

"#;

const TAIL: &str = r#"

\end{document}
"#;

fn export_pdf_to_png(data: Vec<u8>, name: &str, path: &PathBuf, width: u32) -> Result<usize> {
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_byte_vec(data, None)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(width as Pixels)
        .set_maximum_height(2000);

    for (i, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image()
            .into_rgb8()
            .save_with_format(path.join(format!("{}-{:04}.png", name,i)), image::ImageFormat::Png) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }
    Ok(document.pages().len() as _)
}



fn render_to_png_batch(latex: &str, name: &str, path: &PathBuf, width: u32) -> Result<usize> {
    let start = std::time::Instant::now();
    let pdf_data: Vec<u8> =
        tectonic::latex_to_pdf(latex).map_err(|e| anyhow!("{}", e.to_string()))?;
    eprintln!(
        "Finish render {} with time cost: {:?}",
        name,
        start.elapsed()
    );

    let start = std::time::Instant::now();
    let pages =  export_pdf_to_png(pdf_data, name, path, width)?;
    eprintln!(
        "Finish convert {} with time cost: {:?}",
        name,
        start.elapsed()
    );

    Ok(pages)
}

fn render_to_png(latex: &str, name: &str, path: &PathBuf, width: u32) -> Result<()> {
    let start = std::time::Instant::now();
    let latex = HEAD.to_string() + latex + TAIL;
    let pdf_data: Vec<u8> =
        tectonic::latex_to_pdf(latex).map_err(|e| anyhow!("{}", e.to_string()))?;
    eprintln!(
        "Finish render {} with time cost: {:?}",
        name,
        start.elapsed()
    );

    let start = std::time::Instant::now();
    export_pdf_to_png(pdf_data, name, path, width)?;
    eprintln!(
        "Finish convert {} with time cost: {:?}",
        name,
        start.elapsed()
    );

    Ok(())
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    render_to_png(
        &cli.latex,
        &cli.name,
        &cli.output,
        cli.width,
    )?;

    Ok(())
}
