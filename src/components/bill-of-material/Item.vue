<script lang="ts" setup>
import { itemInfo } from "../../Craft"

const props = defineProps<{
    item_id: number
}>()

const { name } = await itemInfo(props.item_id)
</script>

<template>
    {{ name }}
</template>