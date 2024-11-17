<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <div class="relative bg-indigo-900">
    <div class="absolute inset-0" v-if="src !== undefined">
      <img class="w-full h-full object-cover" :src="src" alt="" />
      <div class="absolute inset-0 bg-indigo-800 mix-blend-multiply" aria-hidden="true" />
    </div>
    <div class="relative max-w-7xl mx-auto py-24 px-4 sm:py-32 sm:px-6 lg:px-8">
      <h1 class="text-4xl font-extrabold tracking-tight text-white sm:text-5xl lg:text-6xl" id="headersection-h1-header">
        <slot name="header"></slot>
      </h1>
      <p class="mt-6 text-xl text-indigo-100 max-w-3xl" id="headersection-p-content"><slot></slot></p>
    </div>
  </div>
</template>

<script>
export default {
  name: "HeaderSection",
  props: ['src']
};
</script>

