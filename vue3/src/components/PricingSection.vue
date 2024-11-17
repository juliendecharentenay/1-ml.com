<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <div>
    <div class="pt-12 sm:pt-16 lg:pt-20">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center">
          <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl lg:text-5xl">Service currently free,</h2>
<!--
          <h2 class="text-3xl font-extrabold text-gray-900 sm:text-4xl lg:text-5xl">Simple, usage based pricing</h2>
          <p class="mt-4 text-xl text-gray-600">Sign-up for a 30 day trial - no credit card required</p -->
        </div>
      </div>
    </div>
    <div class="mt-8 pb-16 sm:mt-12 sm:pb-20 lg:pb-28">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="max-w-lg mx-auto rounded-lg shadow-lg overflow-hidden lg:max-w-none lg:flex">
          <div class="flex-1 bg-gray-50 px-6 py-8 lg:p-12">
            <h3 class="text-2xl font-extrabold text-gray-600 sm:text-3xl">Due to low usage and operating costs.</h3>
            <p class="mt-6 text-base text-gray-400">If usage and operating costs were to increase, a subscription fee would be considered or the service would be stopped...</p>
            <div class="mt-8">
              <div class="flex items-center">
              <h4 class="flex-shrink-0 pr-4 bg-white text-sm tracking-wider font-semibold uppercase text-indigo-400">What's included</h4>
                <div class="flex-1 border-t-2 border-gray-200" />
              </div>
              <ul role="list" class="mt-8 space-y-5 lg:space-y-0 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:gap-y-5">
                <li v-for="feature in ['Your domain prefix', 'Up to 100 masked email addresses', 'Email block, forward, forward as text control', 'Up to 1000 emails forwarded per month']" 
                    :key="feature" class="flex items-start lg:col-span-1">
                  <div class="flex-shrink-0">
                    <CheckCircleIcon class="h-5 w-5 text-green-400" aria-hidden="true" />
                  </div>
                  <p class="ml-3 text-sm text-gray-500">
                    {{ feature }}
                  </p>
                </li>
              </ul>
            </div>
          </div>
          <div class="py-8 px-6 text-center bg-gray-100 lg:flex-shrink-0 lg:flex lg:flex-col lg:justify-center lg:p-12">
            <p class="text-lg leading-6 font-medium text-gray-800">Currently free</p>
            <!-- <p class="leading-6 font-normal text-gray-600">After launch:</p> 
            <p class="mt-4 text-lg leading-6 font-medium text-gray-600">Pay monthly</p>
            <div class="mt-4 flex items-center justify-center text-5xl font-extrabold text-gray-600">
              <span> £3 </span>
              <span class="ml-3 text-xl font-medium text-gray-500"> GBP </span>
            </div>
-->
            <p class="mt-4 text-sm">
              <a href="#!" class="font-medium text-gray-500 underline" @click="$emit('view_terms')" id="pricingsection-a-readterms"> Read our service terms </a>
            </p>
            <div class="mt-6">
              <div class="rounded-md shadow">
                <a href="#!" class="flex items-center justify-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-white bg-gray-600 hover:bg-gray-900" @click="$emit('start_trial');" id="pricingsection-a-starttrial"> Sign up </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { CheckCircleIcon } from '@heroicons/vue/24/solid'

export default {
  name: "PricingSection",
  components: {
    CheckCircleIcon,
  },
  emits: ['view_terms', 'start_trial'],
  data: function() {
    return {
    };
  },
};
</script>
