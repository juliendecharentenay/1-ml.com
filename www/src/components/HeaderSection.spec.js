import { mount } from '@vue/test-utils';
import HeaderSection from './HeaderSection.vue';

describe('components/HeaderSection.vue', () => {
  it('renders', () => {
    const wrapper = mount(HeaderSection, {
      slots: {
        default: 'Test main content',
        header: 'Test header content',
      }
    });
    expect(wrapper.find('#headersection-h1-header').text()).toContain('Test header content');
    expect(wrapper.find('#headersection-p-content').text()).toContain('Test main content');
  });
});
