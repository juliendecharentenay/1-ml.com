<template>
  <div class="py-12 sm:py-16 lg:py-20">
    <div class="px-6 max-w-5xl mx-auto">
      <div class="text-left text-lg text-gray-500">
        <h2 class="text-4xl font-bold text-gray-900">
        Privacy Statement
        </h2>

        <dl class="mt-12 space-y-10 sm:space-y-0 sm:grid sm:grid-cols-2 sm:gap-x-6 sm:gap-y-12 lg:grid-cols-3 lg:gap-x-8">
          <div class="relative">
            <dt>
              <ExclamationTriangleIcon class="absolute h-6 w-6 text-orange-800" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Privacy Statement</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            <div>We only collect the minimum amount to information required for providing the service and
            reporting on its uptake. Users are our customers, not the product. The service is built 
            for transparence. </div>
            <div class="mt-2 text-gray-600">#BuildingInPublic</div>
            </dd>
          </div>
    
          <div class="relative">
            <dt>
              <ExclamationTriangleIcon class="absolute h-6 w-6 text-orange-800" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Email Privacy</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            Your emails are stored for 7 days before being automatically deleted. We do not read
            your email's content.
            </dd>
          </div>
    
          <div class="relative">
            <dt>
              <ExclamationTriangleIcon class="absolute h-6 w-6 text-orange-800" />
              <p class="ml-9 text-lg leading-6 font-medium text-gray-900">Analytics</p>
            </dt>
            <dd class="mt-2 ml-9 text-base text-gray-500">
            We collect website traffic using <a href="https://plausible.io">plausible.io</a>, a privacy
            centered analytics. We collect and report on service usage statistics as shown here.
            </dd>
          </div>
    
        </dl>
      </div>
    </div>
  </div>
</template>
<script>
import { ExclamationTriangleIcon } from '@heroicons/vue/24/outline'
export default {
  name: "PrivacySection",
  components: {
    ExclamationTriangleIcon,
  }
};
</script>
