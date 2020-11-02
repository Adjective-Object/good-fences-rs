import { sleep, sync } from '../index'

describe('sync function from native code', () => {
  it('adds 100 to the passed in value', () => {
    const fixture = 42
    expect(sync(fixture)).toBe(fixture + 100)
  })
})

describe('sleep function from native code', () => {
  it('returns the sleep time * 2', async () => {
    const timeToSleep = 200
    const value = await sleep(timeToSleep)
    expect(value).toBe(timeToSleep * 2)
  })
})
