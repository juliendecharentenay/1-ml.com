import { mount } from '@vue/test-utils';
import HeaderElement from './HeaderElement.vue';

describe('components/HeaderElement.vue', () => {
  it('renders', async () => {
    const wrapper = mount(HeaderElement);

    expect(wrapper.emitted()).not.toHaveProperty('to_signin');
    expect(wrapper.emitted()).not.toHaveProperty('to_signup');
    expect(wrapper.emitted()).not.toHaveProperty('to_why');
    expect(wrapper.emitted()).not.toHaveProperty('to_get_started');
    expect(wrapper.emitted()).not.toHaveProperty('to_pricing');
    expect(wrapper.emitted()).not.toHaveProperty('to_terms');

    await wrapper.find('#headerelement-a-to_signin').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_signin');

    await wrapper.find('#headerelement-a-to_signup').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_signup');

    await wrapper.find('#headerelement-a-to_why').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_why');

    await wrapper.find('#headerelement-a-to_get_started').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_get_started');

    await wrapper.find('#headerelement-a-to_pricing').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_pricing');

    await wrapper.find('#headerelement-a-to_terms').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_terms');

    await wrapper.find('#headerelement-a-to_privacy').trigger('click');
    expect(wrapper.emitted()).toHaveProperty('to_privacy');
    
  });
});

