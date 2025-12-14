import React from 'react';
import {render, screen} from '@testing-library/react-native';
import App from '../src/App';

describe('App', () => {
  it('renders correctly', () => {
    render(<App />);
    
    expect(screen.getByText('Kilter Board App')).toBeTruthy();
    expect(screen.getByText('Mobile Application')).toBeTruthy();
  });
});