data "archive_file" "this" {
  type        = "zip"
  source_dir  = "${path.module}/../../functions"
  excludes    = [".venv", "__pycache__", "local.settings.json", "local.settings.json.template"]
  output_path = "${path.module}/functionapp.zip"
}
