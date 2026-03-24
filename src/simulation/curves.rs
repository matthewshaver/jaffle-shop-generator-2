// Curves module - the curve logic is now embedded directly in Day methods
// in time.rs. This file is kept for potential future extension.

// AnnualCurve: (cos(x) + 1) / 10 + 0.8 where x = day_of_year / 365 * 2*PI
// WeekendCurve: 0.6 for weekends, 1.0 for weekdays
// GrowthCurve: 1 + ((year - 2016) * 12 + month) / 12 * 0.2
