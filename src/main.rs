use anyhow::Result;
use std::env;
use std::fs;

fn main() {
  // 设置默认日志级别
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }
  env_logger::init();

  let result = create_files("./data.csv");
  if let Err(e) = result {
    log::error!("Error: {}", e);
  }
}

fn get_template_file_name() -> Result<String> {
  // 遍历当前目录
  let entries = fs::read_dir(".")?;
  for entry in entries {
    let path = entry?.path();
    // 如果文件是 template 或 template.* 任意后缀名
    let file_name = path
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();
    if path.is_file() && file_name.starts_with("template") {
      return Ok(file_name.to_string());
    }
  }
  Err(anyhow::anyhow!("No template file found"))
}

fn create_files(data_path: &str) -> Result<()> {
  let template_file_name = get_template_file_name()?;
  let template_path = format!("./{}", template_file_name);
  let template_content = fs::read_to_string(&template_path)?;
  let extension = std::path::Path::new(&template_path)
    .extension()
    .and_then(|ext| ext.to_str())
    .unwrap_or("");

  let mut reader = csv::Reader::from_path(data_path)?;

  // 获取 CSV 的表头作为变量名
  let headers = reader.headers()?.clone();

  for result in reader.records() {
    let record = result?;
    let mut content = template_content.clone();

    // 遍历每个表头和对应的值，进行替换
    for (i, header) in headers.iter().enumerate() {
      let value = &record[i];
      content = content.replace(&format!("${{{}}}", header), value);
    }

    // 使用第一列作为文件名
    let file_name = &record[0];
    let output_path = {
      if extension.is_empty() {
        file_name.to_string()
      } else {
        format!("{}.{}", file_name, extension)
      }
    };

    fs::write(&output_path, content)?;
    log::info!("Created file: {}", output_path);
  }
  Ok(())
}
