<template>
  <div class="flex flex-row items-start">
    <div class="text-2xl text-indigo-600 font-extrabold">
      <slot name="counter"></slot>
    </div>
    <div>
      <dt>
        <p class="ml-9 text-lg font-medium text-gray-900">
          <slot name="header"></slot>
        </p>
      </dt>
      <dd class="ml-9 text-base">
        <slot name="description"></slot>
      </dd>
    </div>
  </div>
</template>
<script>
export default {
  name: 'HowToStep',
};
</script>
