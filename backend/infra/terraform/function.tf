resource "azurerm_service_plan" "this" {
  name                = format("%s-appserviceplan", local.prefix)
  location            = data.azurerm_resource_group.this.location
  resource_group_name = data.azurerm_resource_group.this.name
  os_type             = "Linux"
  sku_name            = "FC1"
}


resource "azurerm_function_app_flex_consumption" "this" {
  name                = format("%s-function", local.prefix)
  resource_group_name = data.azurerm_resource_group.this.name
  location            = data.azurerm_resource_group.this.location
  service_plan_id     = azurerm_service_plan.this.id

  storage_container_type = "blobContainer"

  storage_container_endpoint  = "${azurerm_storage_account.this.primary_blob_endpoint}${azurerm_storage_container.this.name}"
  storage_authentication_type = "StorageAccountConnectionString"
  storage_access_key          = azurerm_storage_account.this.primary_access_key
  runtime_name                = "python"
  runtime_version             = "3.12"
  instance_memory_in_mb       = 512

  app_settings = {
    application_insights_connection_string = azurerm_application_insights.this.connection_string
  }

  site_config {}
}

