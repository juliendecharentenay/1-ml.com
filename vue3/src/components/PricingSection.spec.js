import { mount } from "@vue/test-utils";
import PricingSection from './PricingSection.vue';

describe('components/PricingSection.vue', () => {
  it('renders', async () => {
    const wrapper = mount(PricingSection);

    await wrapper.find('#pricingsection-a-readterms').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('view_terms');

    await wrapper.find('#pricingsection-a-starttrial').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('start_trial');

  });
});
