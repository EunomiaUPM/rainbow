import {generateFiles} from 'fumadocs-openapi';
import * as mod from '../lib/openapi';

const {openapi} = mod
void generateFiles({
    input: openapi,
    output: './content/docs/catalog/openapi',
    includeDescription: true,
    per: 'tag',
});