import { Item, Jobs, Recipe, RecipeInfo, RecipeLevel, RecipeRequirements } from "../../Craft"
import { useDesignerStore } from "../../store"

const designerStore = useDesignerStore()

const jobMapsZh: { [key: string]: Jobs } = {
    '木工': Jobs.Carpenter,
    '锻冶': Jobs.Blacksmith,
    '铸甲': Jobs.Armorer,
    '雕金': Jobs.Goldsmith,
    '制革': Jobs.Leatherworker,
    '裁缝': Jobs.Weaver,
    '炼金': Jobs.Alchemist,
    '烹调': Jobs.Culinarian,
}
const jobMapsEn: { [key: string]: Jobs } = {
    'Woodworking': Jobs.Carpenter,
    'Smithing': Jobs.Blacksmith,
    'Armorcraft': Jobs.Armorer,
    'Goldsmithing': Jobs.Goldsmith,
    'Leatherworking': Jobs.Leatherworker,
    'Clothcraft': Jobs.Weaver,
    'Alchemy': Jobs.Alchemist,
    'Cooking': Jobs.Culinarian,
}
const jobMapsJa: { [key: string]: Jobs } = {
    '木工': Jobs.Carpenter,
    '鍛冶': Jobs.Blacksmith,
    '甲冑': Jobs.Armorer,
    '彫金': Jobs.Goldsmith,
    '革細工': Jobs.Leatherworker,
    '裁縫': Jobs.Weaver,
    '錬金': Jobs.Alchemist,
    '調理': Jobs.Culinarian,
}
const jobMapsDe: { [key: string]: Jobs } = {
    'Zimmerer': Jobs.Carpenter,
    'Grobschmied': Jobs.Blacksmith,
    'Plattner': Jobs.Armorer,
    'Goldschmied': Jobs.Goldsmith,
    'Gerber': Jobs.Leatherworker,
    'Weber': Jobs.Weaver,
    'Alchemist': Jobs.Alchemist,
    'Gourmet': Jobs.Culinarian,
}
const jobMapsFr: { [key: string]: Jobs } = {
    'Menuiserie': Jobs.Carpenter,
    'Métallurgie': Jobs.Blacksmith,
    'Armurerie': Jobs.Armorer,
    'Orfèvrerie': Jobs.Goldsmith,
    'Tannerie': Jobs.Leatherworker,
    'Couture': Jobs.Weaver,
    'Alchimie': Jobs.Alchemist,
    'Cuisine': Jobs.Culinarian,
}

export function craftTypeTojobs(craftType: string): Jobs {
    return jobMapsZh[craftType]
        ?? jobMapsEn[craftType]
        ?? jobMapsJa[craftType]
        ?? jobMapsDe[craftType]
        ?? jobMapsFr[craftType]
        ?? Jobs.Culinarian
}

export const selectRecipe = (recipe: Recipe, recipeLevel: RecipeLevel, materialQualityFactor: number, requirements: RecipeRequirements, item: Item, craftType: string, simulatorMode: boolean) => {
    designerStore.selectRecipe({
        job: craftTypeTojobs(craftType),
        item,
        recipe,
        materialQualityFactor,
        recipeLevel,
        requirements,
        simulatorMode,
    })
}