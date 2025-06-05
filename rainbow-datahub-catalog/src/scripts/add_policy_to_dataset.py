
# from datahub.emitter.mce_builder import make_dataset_urn
# from datahub.ingestion.graph.client import DataHubGraph, DataHubGraphConfig
# from datahub.specific.dataset import DatasetPatchBuilder
# import logging
# import sys

# # Configurar logging
# logging.basicConfig(
#     level=logging.INFO,
#     format='%(asctime)s - %(levelname)s - %(message)s'
# )
# logger = logging.getLogger(__name__)

# def add_policy_to_dataset(dataset_urn: str, property_name: str, property_value: str) -> bool:
#     """
#     Añade una custom property a un dataset en DataHub.
    
#     Args:
#         dataset_urn (str): El URN del dataset al que se le añadirá la property
#         property_name (str): Nombre de la property a añadir
#         property_value (str): Valor de la property a añadir
    
#     Returns:
#         bool: True si la operación fue exitosa, False en caso contrario
#     """
#     try:
#         # Cliente drugs
#         datahub_client = DataHubGraph(
#             DataHubGraphConfig(
#                 server="http://localhost:8086",
#                 token="eyJhbGciOiJIUzI1NiJ9.eyJhY3RvclR5cGUiOiJVU0VSIiwiYWN0b3JJZCI6ImRydWdzQGRydWdzLmNvbSIsInR5cGUiOiJQRVJTT05BTCIsInZlcnNpb24iOiIyIiwianRpIjoiYmZkMTA5MjYtODE0MC00ODk1LTliNTgtNTMzMWMxMjY2MWMwIiwic3ViIjoiZHJ1Z3NAZHJ1Z3MuY29tIiwiZXhwIjoxNzUxNTM3NzQxLCJpc3MiOiJkYXRhaHViLW1ldGFkYXRhLXNlcnZpY2UifQ.VvTiXmU98Hnurdg9g_xINtz3zyvtM2SpF6Ad23h7kJM"
#             )
#         )

#         logger.info(f"Usando dataset URN: {dataset_urn}")
        
#         # Crear Dataset Patch para añadir la custom property
#         patch_builder = DatasetPatchBuilder(dataset_urn)
#         patch_builder.add_policy_to_dataset(property_name, property_value)
#         patch_mcps = patch_builder.build()
        
#         # Emitir Dataset Patch
#         for patch_mcp in patch_mcps:
#             logger.info(f"Enviando patch: {patch_mcp}")
#             datahub_client.emit(patch_mcp)
            
#         logger.info(f"Custom property '{property_name}' añadida exitosamente")
#         return True
        
#     except Exception as e:
#         logger.error(f"Error al añadir custom property: {str(e)}")
#         return False

# # Punto de entrada para llamadas desde Rust
# if __name__ == "__main__":
#     if len(sys.argv) != 4:
#         print("Uso: python3 add_policy_to_dataset.py <dataset_urn> <property_name> <property_value>")
#         sys.exit(1)
    
#     dataset_urn = sys.argv[1]
#     property_name = sys.argv[2]
#     property_value = sys.argv[3]
    
#     success = add_policy_to_dataset(dataset_urn, property_name, property_value)
#     sys.exit(0 if success else 1)


# ====================================================


# Inlined from /metadata-ingestion/examples/library/dataset_add_remove_custom_properties_patch.py
# from datahub.emitter.mce_builder import make_dataset_urn
# from datahub.ingestion.graph.client import DataHubGraph, DataHubGraphConfig
# from datahub.specific.dataset import DatasetPatchBuilder

# # Create DataHub Client
# datahub_client = DataHubGraph(DataHubGraphConfig(server="http://localhost:8080"))

# # Create Dataset URN
# dataset_urn = make_dataset_urn(platform="hive", name="fct_users_created", env="PROD")

# # Create Dataset Patch to Add + Remove Custom Properties
# patch_builder = DatasetPatchBuilder(dataset_urn)
# patch_builder.add_custom_property("cluster_name", "datahubproject.acryl.io")
# patch_builder.remove_custom_property("retention_time")
# patch_mcps = patch_builder.build()

# # Emit Dataset Patch
# for patch_mcp in patch_mcps:
#     datahub_client.emit(patch_mcp)
