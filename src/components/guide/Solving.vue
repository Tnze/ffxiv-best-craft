<script setup lang="ts">
import { ElText } from 'element-plus';
import { useGuideStore } from '../../store';

const guideStore = useGuideStore();

function selectSolver() {
    if (guideStore.recipe == null || guideStore.craftTypeAttr == null)
        return
    if (guideStore.craftTypeAttr.level - guideStore.recipe.job_level > 10) {
        // 可能可以使用工匠的神速技巧
    }
    
}

</script>

<template>
    <div class="container">
        <el-text>Solver</el-text>
    </div>
</template>

<style scoped>
.containter {
    display: flex;
}
</style>
