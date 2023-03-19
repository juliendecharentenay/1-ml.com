<template>
  <a href="#/" 
     class="group flex items-center px-2 py-2 text-base leading-5 font-medium rounded-md"
     :class="{'bg-gray-200 text-gray-900': current && ! disable, 
              'text-gray-700': ! current || disable, 
              'hover:text-gray-900 hover:bg-gray-50': ! current && ! disable}"
     id="sidebaritem-a-content"
     >
     <div class="text-gray-500 mr-3 flex-shrink-0 h-6 w-6" id="sidebaritem-div-icon">
       <slot name="icon"></slot>
     </div>
     <slot></slot>
  </a>
</template>
<script>
export default {
  name: "SidebarItem",
  props: {
    current: {
      type: Boolean,
      default: false,
    },
    disable: {
      type: Boolean,
      default: false,
    }
  },
};
</script>
