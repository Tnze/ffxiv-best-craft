<script setup lang="ts">
import { ref } from 'vue'
import { Attributes, Jobs, Status } from '../../Craft'
import ActionPanel from './ActionPanel.vue'
import Condition from './Condition.vue'
import Split from './Split.vue';

const props = defineProps<{
    itemName: string;
    attributes: Attributes;
    status: Status;
}>()



</script>

<template>
    <el-container>
        <el-header>
            <el-row :gutter="20">
                <el-col :span="18">
                    <h1>{{ itemName }}</h1>
                </el-col>
                <el-col :span="6">
                </el-col>
            </el-row>
        </el-header>
        <el-main>
            <Split>
                <template #upper>
                    <el-row :gutter="0">
                        <el-col id="condition" :span="4">
                            耐久
                            <br />
                            <Condition :cond="status.condition" />
                        </el-col>
                        <el-col :span="8">进度条</el-col>
                    </el-row>
                    <el-row>
                        <el-scrollbar></el-scrollbar>
                    </el-row>
                </template>
                <template #lower>
                    <el-scrollbar>
                        <ActionPanel :job="Jobs.Armorer" #lower />
                    </el-scrollbar>
                </template>
            </Split>
        </el-main>
    </el-container>
</template>

<style scoped>
.el-header {
    text-align: left;
}

#condition {
    text-align: center;
}
.el-footer {
    padding: 0;
    height: 100px;
}
</style>
