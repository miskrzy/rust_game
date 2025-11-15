resource "azurerm_service_plan" "this" {
  name                = format("%s-appserviceplan", local.prefix)
  location            = data.azurerm_resource_group.this.location
  resource_group_name = data.azurerm_resource_group.this.name
  os_type             = "Linux"
  sku_name            = "Y1"
}


resource "azurerm_linux_function_app" "this" {
  name                = format("%s-function", local.prefix)
  resource_group_name = data.azurerm_resource_group.this.name
  location            = data.azurerm_resource_group.this.location

  storage_account_name       = azurerm_storage_account.this.name
  storage_account_access_key = azurerm_storage_account.this.primary_access_key
  service_plan_id            = azurerm_service_plan.this.id

  # https_only      = true
  zip_deploy_file = data.archive_file.this.output_path

  site_config {
    application_insights_connection_string = azurerm_application_insights.this.connection_string
  }

  app_settings = {
    FUNCTIONS_WORKER_RUNTIME = "python"
  }
}

