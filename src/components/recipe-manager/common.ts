import { Item, Jobs, Recipe, RecipeLevel, RecipeRequirements } from "../../Craft"
import { useDesignerStore } from "../../store"

const designerStore = useDesignerStore()

const jobMaps: { [key: string]: Jobs } = {
    '木工': Jobs.Carpenter,
    '锻冶': Jobs.Blacksmith,
    '铸甲': Jobs.Armorer,
    '雕金': Jobs.Goldsmith,
    '制革': Jobs.Leatherworker,
    '裁缝': Jobs.Weaver,
    '炼金': Jobs.Alchemist,
    '烹调': Jobs.Culinarian,
}

export const selectRecipe = (recipe: Recipe, recipeLevel: RecipeLevel, requirements: RecipeRequirements, item: Item, craftType: string, simulatorMode: boolean) => {
    console.log("select recipe", recipe, recipeLevel, requirements)
    designerStore.selectRecipe({
        job: jobMaps[craftType] ?? Jobs.Culinarian,
        item,
        recipe,
        recipeLevel,
        requirements,
        simulatorMode,
    })
}