import { mount } from '@vue/test-utils';
import {
  it, expect, beforeAll, vitest,
} from 'vitest';

import App from '../src/App.vue';

beforeAll(() => {
  // @ts-ignore
  global.fetch = vitest.fn(() => Promise.resolve({
    json: () => Promise.resolve({}),
  }));
});

it('renders', () => {
  const wrapper = mount(App);
  expect(wrapper.html()).toMatchSnapshot();
});
