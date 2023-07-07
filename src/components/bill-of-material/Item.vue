<script lang="ts" setup>
import { useSettingsStore } from '../../store';

const settingStore = useSettingsStore()

const props = defineProps<{
    item_id: number
}>()

const { name } = await settingStore.getDataSource.itemInfo(props.item_id)
</script>

<template>
    {{ name }}
</template>