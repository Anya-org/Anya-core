const REQUIRED_LABELS = [
  /\[AIR-\d\]/, 
  /\[AIS-\d\]/,
  /\[BPC-\d\]/
];

const LABEL_PATTERNS = {
  AIR: /\[AIR-[3-4]\]/,
  AIS: /\[AIS-[3-4]\]/,
  BPC: /\[BPC-[3-4]\]/,
  RES: /\[RES-[3-4]\]/
};

// Enhanced label validation using compliance matrix [Ref: https://javascript.plainenglish.io/how-to-use-the-dry-principle-properly-21fd354b48c3]
const LABEL_REQUIREMENTS = {
  CORE: [
    /\[AIR-[3-4]\]/,
    /\[AIS-[3-4]\]/,
    /\[BPC-[3-4]\]/
  ],
  PROJECTS: [
    /\[RES-[2-3]\]/,
    /\[AIP-[2-3]\]/
  ]
};

function validateLabels(code, context) {
  const missing = LABEL_REQUIREMENTS[context].filter(re => !re.test(code));
  return {
    compliant: missing.length === 0,
    missingLabels: missing.map(m => m.source),
    validationDate: new Date().toISOString()
  };
}

function validateComplianceLabels(code) {
  return Object.entries(LABEL_PATTERNS).every(([key, regex]) => {
    const match = code.match(regex);
    if (!match) console.error(`Missing ${key} compliance label`);
    return !!match;
  });
} 