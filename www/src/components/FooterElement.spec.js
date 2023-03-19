import { mount } from '@vue/test-utils';
import FooterElement from './FooterElement.vue';

describe('components/FooterElement.vue', () => {
  it('renders', () => {
    const wrapper = mount(FooterElement);
    expect(wrapper.find('#footerelement-p-copyright').text()).toContain('de Charentenay');
  });
});
