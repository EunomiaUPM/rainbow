const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

// With JSDoc @type annotations, IDEs can provide config autocompletion
/** @type {import('@docusaurus/types').DocusaurusConfig} */
(module.exports = {
    title: 'My Site',
    tagline: 'Dinosaurs are cool',
    url: 'https://ging.github.io/rainbow/',
    baseUrl: '/rainbow/',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    favicon: 'img/favicon.ico',
    organizationName: 'facebook', // Usually your GitHub org/user name.
    projectName: 'docusaurus', // Usually your repo name.

    presets: [
        [
            '@docusaurus/preset-classic',
            /** @type {import('@docusaurus/preset-classic').Options} */
            ({
                docs: {
                    sidebarPath: require.resolve('./sidebars.js'),
                    // Please change this to your repo.
                    editUrl: 'https://github.com/facebook/docusaurus/edit/main/website/',
                },
                blog: {
                    showReadingTime: true,
                    // Please change this to your repo.
                    editUrl:
                        'https://github.com/facebook/docusaurus/edit/main/website/blog/',
                },
                theme: {
                    customCss: require.resolve('./src/css/custom.css'),
                },
            }),
        ],
    ],

    themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
        ({
            navbar: {
                title: 'Rainbow - Dataspace Transfer Protocol',
                logo: {
                    alt: 'Rainbow Logo',
                    src: 'img/logo_rainbow.svg',
                },
                items: [
                    {
                        href: 'https://github.com/ging/rainbow',
                        label: 'GitHub',
                        position: 'right',
                    },
                ],
            },
            footer: {
                copyright: `GNU - ${new Date().getFullYear()} Eunomia Rainbow - Universidad Politécnica de Madrid.`,
            },
            prism: {
                theme: lightCodeTheme,
                darkTheme: darkCodeTheme,
            },
        }),
});
