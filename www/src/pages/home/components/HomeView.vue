<template>
      <!-- Page title & actions -->
      <div class="hidden sm:block border-b border-gray-200 px-4 py-2 sm:flex sm:items-center sm:justify-between sm:px-6 lg:px-8">
        <div class="flex-1 min-w-0">
          <h1 class="text-lg font-medium leading-6 text-gray-900 sm:truncate"> Home </h1>
        </div>
      </div>

      <!-- No email message -->
      <div class="mt-8" v-if="emails.length === 0">
        <p class="px-6 text-gray-500">
        No emails have been used on your account. Add something to explain how it works...
        </p>
      </div>

      <!-- table (small breakpoint and up) -->
      <div class="mt-4" v-if="emails.length > 0">
        <div class="align-middle inline-block min-w-full border-b border-gray-200">
          <table class="min-w-full">
            <thead>
              <tr class="border-t border-gray-200">
                <th class="px-6 py-3 border-b border-gray-200 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" scope="col">
                  <a href="#/" class="flex" @click="sort_by('address')">
                  <span class="lg:pl-2">Address</span>
                  <ArrowNarrowUpIcon class="w-5 h-5" v-if="sorted_by('address') === 1" />
                  <ArrowNarrowDownIcon class="w-5 h-5" v-if="sorted_by('address') === -1" />
                  </a>
                </th>
                <!-- <th class="px-6 py-3 border-b border-gray-200 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider" scope="col">Activity</th> -->
                <th class="pr-6 md:pr-12 lg:pr-24 py-3 border-b border-gray-200 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider" scope="col">
                  <a href="#/" class="flex justify-center" @click="sort_by('status')">
                  Status
                  <ArrowNarrowUpIcon class="w-5 h-5" v-if="sorted_by('status') === 1" />
                  <ArrowNarrowDownIcon class="w-5 h-5" v-if="sorted_by('status') === -1" />
                  </a>
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-100 ">
              <tr v-for="email in emails"
                  :key="email.email"
                  >
                <td class="px-2 sm:px-4 md:px-6 py-1 md:py-2 max-w-0 w-full text-sm font-medium text-gray-900">
                  <div class="lg:pl-2 truncate">
                    {{ email.email }}
                  </div>
                </td>
                <!-- <td class="px-6 py-3 text-sm text-gray-500 font-medium">
                  7/25
                </td> -->
                <td class="pr-6 md:pr-12 lg:pr-24 py-1 md:py-2 whitespace-nowrap text-center text-sm font-medium">
                  <div>
                    <select class="mt-1 block pl-3 pr-10 py-1 text-sm sm-gray-300 ring-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
                            @change="set_status(email.email, $event.target.value)"
                            >
                      <option value="Forward" :selected="email.status === 'Forward'"
                              >Forward</option>
                      <option value="ForwardAsText" :selected="email.status === 'ForwardAsText'"
                              >Forward as text</option>
                      <option value="Block" :selected="email.status === 'Block'"
                              >Block</option>
                    </select>
                  </div>
                </td>
              </tr>

              <!-- More projects... -->
            </tbody>
          </table>
        </div>
      </div>
</template>
<script>
import { get, patch } from "@/js/api.js";
import { ArrowNarrowUpIcon, ArrowNarrowDownIcon, } from "@heroicons/vue/outline";

export default {
  name: "HomeView",
  props: ['user', 'search'],
  emits: ["loading", "on_error"],
  components: {
    ArrowNarrowUpIcon, ArrowNarrowDownIcon,
  },
  data: function() {
    return {
      p_emails: [],
      sort: [],
    };
  },
  mounted: function() {
    try {
      get(this.user, "/api/email")
      .then((r) => {this.p_emails = r;})
      .catch((e) => {this.on_error("Error when querying list of emails", e);});
    } catch (e) {
      this.on_error("Error in mounted", e);
    }
  },
  computed: {
    emails: function() {
      try {
        let emails = this.p_emails;
        if (this.search !== null && this.search !== "") {
          let re = new RegExp(this.search, "i");
          emails = emails.filter((e) => re.test(e.email));
        }
        for (const sort of this.sort) {
          if (sort.label === "address") {
            emails.sort((a, b) => a.email.localeCompare(b.email));
          } else if (sort.label === "status") {
            emails.sort((a, b) => a.status.localeCompare(b.status));
          } else {
            throw new Error(`Sorting label ${sort.label} is not supported`);
          }
          if (sort.direction === -1) { emails.reverse(); }
        }
        return emails;
      } catch(e) {
        this.on_error("Error in emails", e);
      }
      return [];
    }
  },
  methods: {
    set_status: function(email, status) {
      try {
        console.log("set_status", email, status);
        patch(this.user, `/api/email/${encodeURIComponent(email)}`, {status})
        .then((r) => { this.p_emails.find((e) => e.email === r.email).status = r.status; })
        .catch((e) => { this.on_error("Error when upading email status", e); });
      } catch(e) {
        this.on_error("Error in set_status", e);
      }
    },
    sort_by: function(label) {
      try {
        let sort = this.sort.find((e) => e.label === label);
        sort = {label, direction: (sort === undefined ? 1 : -1*sort.direction)};
        this.sort = [...this.sort.filter((e) => e.label !== label), sort];
        console.log("Sort = ", this.sort);
      } catch (e) {
        this.on_error("Error in sort_by", e);
      }
    },
    sorted_by: function(label) {
      try {
        if (this.sort.length > 0) {
          let sort = this.sort[this.sort.length-1];
          if (sort.label === label) {
            return sort.direction;
          }
        }
      } catch (e) {
        this.on_error("Error in sorted_by", e);
      }
      return null;
    },
    on_error: function(message, error) {
      this.$emit('on_error', { message, error });
    }
  }
};
</script>
