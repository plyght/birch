# Quick Fix for Dashboard

## Issues Fixed

1. Added `@heroicons/react` package for icons
2. Updated home page to redirect properly (authenticated → dashboard, not authenticated → login)
3. Set dashboard to run on port 3001

## Steps to Fix

```bash
# 1. Install dependencies (includes heroicons)
cd apps/web
bun install
cd ../..

# 2. Restart dev servers
bun run dev
```

## What Changed

- **package.json**: Added `@heroicons/react` dependency and port 3001 for dev server
- **apps/web/src/app/page.tsx**: Now redirects to `/dashboard` or `/login` instead of showing old form
- All sidebar icons will now display correctly
- Links are now clickable and navigate properly

## Testing

1. Go to `http://localhost:3001`
2. Should redirect to `/login` if not authenticated
3. After login, redirects to `/dashboard`
4. Sidebar should show all icons and be clickable
5. Click any menu item to navigate

## Ports

- API: http://localhost:3000
- Dashboard: http://localhost:3001
- Redis: localhost:6379

