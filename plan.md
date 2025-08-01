# Recipe Scoring Strategy Plan

## Problem Statement
Complex recipe pages contain extensive noise: navigation menus, comments, related content, advertisements, and promotional material. Current scoring needs improvement to isolate actual recipe content from this noise.

## Core Scoring Strategies

### 1. Content Isolation
- **Recipe Schema Detection**: Heavily weight content within structured recipe markup
- **Section Header Boost**: Increase scores for content following clear headers ("Ingredients", "Instructions", "Marinade", "Method")
- **Title Proximity**: Apply positional weighting - content closer to recipe title scores higher
- **Container Recognition**: Identify and boost recipe-specific containers/divs

### 2. Content Density Analysis
- **Ingredient Clustering**: Score regions with high concentrations of ingredient corpus matches
- **Measurement Patterns**: Boost areas with cooking measurements (cups, tbsp, tsp, etc.)
- **Cooking Verb Clusters**: Identify regions dense with recipe instruction verbs
- **Sliding Window**: Use moving window analysis to find recipe-dense content blocks

### 3. Structural Pattern Recognition
- **List Detection**: Boost bullet-pointed/numbered lists matching ingredient patterns
- **Instruction Sequences**: Recognize numbered cooking steps and method lists  
- **Metadata Patterns**: Identify recipe metadata (prep time, cook time, servings, ratings)
- **Quantity + Unit Combinations**: Score "1 cup flour", "2 tbsp oil" patterns highly

### 4. Noise Filtering (Anti-patterns)
- **Navigation Penalty**: Heavily penalize menu items, breadcrumbs, site navigation
- **Comment Sections**: Filter out user comments and reviews
- **Related Content**: Penalize "You might also like" and recipe suggestions
- **Social/Sharing**: Filter social media buttons and sharing widgets
- **Advertisement**: Detect and penalize promotional content

### 5. Context-Aware Scoring
- **Topic Consistency**: Ingredients should cluster together, instructions should flow
- **Section Boundaries**: Respect natural breaks between ingredients/instructions
- **Content Flow**: Logical progression from ingredients → method → notes
- **Relevance Decay**: Score decreases with distance from main recipe content

## Implementation Approach

### Phase 1: Basic Pattern Matching
- Implement ingredient corpus matching with position weighting
- Add measurement pattern recognition
- Create section header detection

### Phase 2: Structural Analysis  
- Build list detection and scoring
- Implement sliding window content density analysis
- Add instruction sequence recognition

### Phase 3: Noise Filtering
- Create anti-pattern detection for navigation/comments
- Implement content container isolation
- Add promotional content filtering

### Phase 4: Advanced Scoring
- Combine multiple signals with weighted scoring
- Implement topic consistency checking
- Add contextual relevance scoring

## Success Metrics
- Clean extraction of ingredients lists from noisy pages
- Accurate instruction sequence identification
- Minimal noise in final parsed output
- Robust handling of various recipe site layouts
