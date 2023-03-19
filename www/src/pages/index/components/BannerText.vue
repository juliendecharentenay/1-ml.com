<template>
<div class="bg-indigo-600">
  <div class="mx-auto max-w-7xl py-3 px-3 sm:px-6 lg:px-8">
    <div class="pr-16 sm:px-16 sm:text-center">
      <p class="font-medium text-white">
        <slot></slot>
      </p>
    </div>
  </div>
</div>
</template>
<script>
export default {
  name: "BannerText",
};
</script>

