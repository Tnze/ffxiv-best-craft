<script setup lang="ts">
import Split from 'split.js'
import { getCurrentInstance, onMounted, ref } from 'vue';

const split0 = ref<HTMLElement | null>(null)
const split1 = ref<HTMLElement | null>(null)

onMounted(() => {
    Split([split0.value!, split1.value!], {
        direction: 'vertical',
        sizes: [80, 20],
        minSize: 0,
    })
})
</script>

<template>
    <div class="split">
        <div ref="split0">
            <slot name="upper"></slot>
        </div>
        <div ref="split1">
            <slot name="lower"></slot>
        </div>
    </div>
</template>

<style>
.split {
    height: 100%;
}
.gutter {
    background-color: #eee;
    background-repeat: no-repeat;
    background-position: 50%;
}

.gutter.gutter-vertical {
    background-image: url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAB4AAAAFAQMAAABo7865AAAABlBMVEVHcEzMzMzyAv2sAAAAAXRSTlMAQObYZgAAABBJREFUeF5jOAMEEAIEEFwAn3kMwcB6I2AAAAAASUVORK5CYII=");
    cursor: row-resize;
}
</style>