<script setup lang="ts">
import { onMounted, ref } from 'vue';

const message = ref('');

const getMessage = async () => {
  const response = await fetch('http://localhost:8080/api/hello').then((res) => res.json());
  message.value = response.message;
};

onMounted(() => {
  getMessage();
});
</script>

<template>
  <p>{{ message }}</p>
</template>

<style scoped>
</style>
