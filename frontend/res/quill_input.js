      // Initialize the Quill editor
      const quill = new Quill('#task-content', {
        theme: 'snow',
        placeholder: 'Write something amazing...',
      });
  
      // Handle the "Show Output" button
      quill.on('text-change', () => {
        // Get the formatted HTML content from the Quill editor
        const htmlContent = quill.root.innerHTML;
  
        // Display the raw HTML in the preformatted block
        document.getElementById('html-output').textContent = htmlContent;
      });