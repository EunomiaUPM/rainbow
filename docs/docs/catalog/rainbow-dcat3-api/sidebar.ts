import type {SidebarsConfig} from "@docusaurus/plugin-content-docs";

const sidebar: SidebarsConfig = {
    apisidebar: [
        {
            type: "doc",
            id: "catalog/rainbow-dcat3-api/rainbow-dcat-3-catalog-api",
        },
        {
            type: "category",
            label: "Catalog",
            items: [
                {
                    type: "doc",
                    id: "catalog/rainbow-dcat3-api/handle-get-catalogs",
                    label: "Retrieves DCAT3 Catalogs",
                    className: "api-method get",
                },
                {
                    type: "doc",
                    id: "catalog/rainbow-dcat3-api/handle-post-catalog",
                    label: "Creates a new DCAT3 Catalog",
                    className: "api-method post",
                },
                {
                    type: "doc",
                    id: "catalog/rainbow-dcat3-api/handle-get-catalog-by-id",
                    label: "Retrieves a DCAT3 Catalog by ID",
                    className: "api-method get",
                },
                {
                    type: "doc",
                    id: "catalog/rainbow-dcat3-api/handle-put-catalog",
                    label: "Updates a DCAT3 Catalog by ID",
                    className: "api-method put",
                },
                {
                    type: "doc",
                    id: "catalog/rainbow-dcat3-api/handle-delete-catalog",
                    label: "Deletes a DCAT3 Catalog by ID",
                    className: "api-method delete",
                },
            ],
        },
    ],
};

export default sidebar.apisidebar;
