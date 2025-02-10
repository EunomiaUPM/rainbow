const {themes} = require('prism-react-renderer');
const lightTheme = themes.github;
const darkTheme = themes.dracula;

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
    title: 'My Site',
    tagline: 'Dinosaurs are cool',
    url: 'http://localhost:3000/',
    baseUrl: '/',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    favicon: '/img/favicon.ico',
    organizationName: 'facebook', // Your GitHub org/user name.
    projectName: 'docusaurus', // Your repo name.

    presets: [['@docusaurus/preset-classic', /** @type {import('@docusaurus/preset-classic').Options} */
        {
            docs: {
                sidebarPath: require.resolve('./sidebars.js'), // Change or add the doc item component for OpenAPI docs:
                docItemComponent: '@theme/ApiItem', // Edit URL (optional)
                editUrl: 'https://github.com/facebook/docusaurus/edit/main/website/', routeBasePath: '/',
            }, theme: {
                customCss: require.resolve('./src/css/custom.css'),
            },
        },],],

    plugins: [['docusaurus-plugin-openapi-docs', {
        id: 'api', // Plugin instance id.
        // IMPORTANT: Change from "classic" to "default" so it matches the docs plugin id.
        docsPluginId: 'default',
        config: {
            catalog: {
                // Verify that the path is correct relative to your project root!
                specPath: './../rainbow-catalog/src/api/rainbow_catalog_api.yaml',
                outputDir: 'docs/catalog/rainbow-dcat3-api',
                sidebarOptions: {
                    groupPathsBy: 'tagGroup',
                },
            },
        },
    },],],

    // IMPORTANT: Uncomment the theme line so the plugin’s styling is applied.
    themes: ['docusaurus-theme-openapi-docs'],

    themeConfig: {
        navbar: {
            title: 'Rainbow - Dataspace Transfer Protocol', logo: {
                alt: 'Rainbow Logo', src: '/img/logo_rainbow.svg',
            }, items: [{
                href: 'https://github.com/ging/rainbow', label: 'GitHub', position: 'right',
            },],
        }, footer: {
            copyright: `GNU - ${new Date().getFullYear()} 
      Eunomia Rainbow - Universidad Politécnica de Madrid.`,
        }, prism: {
            theme: lightTheme, darkTheme: darkTheme,
        },
    },
};
