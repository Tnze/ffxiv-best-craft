<script setup lang="ts">
import { onActivated } from 'vue';
import RecipePanel from './RecipePanel.vue';

const emit = defineEmits<{
    (e: 'setTitle', title: string): void;
}>();
onActivated(() => emit('setTitle', 'favorite-recipes'));

</script>
<template>
    <RecipePanel :limitFavorites="true" />
</template>
<style scoped></style>